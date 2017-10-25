use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn sgdt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SGDT, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 1, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 66, 1], OperandSize::Word)
}

fn sgdt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SGDT, operand1: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 4, 214], OperandSize::Dword)
}

fn sgdt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SGDT, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 652518130, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 132, 216, 242, 162, 228, 38], OperandSize::Qword)
}

