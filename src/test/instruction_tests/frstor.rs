use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn frstor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FRSTOR, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 33], OperandSize::Word)
}

fn frstor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FRSTOR, operand1: Some(IndirectScaledDisplaced(ESI, Four, 443621469, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 36, 181, 93, 32, 113, 26], OperandSize::Dword)
}

fn frstor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FRSTOR, operand1: Some(Indirect(RDX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 34], OperandSize::Qword)
}

