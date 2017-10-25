use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn xrstor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTOR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 22367, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 169, 95, 87], OperandSize::Word)
}

fn xrstor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTOR, operand1: Some(IndirectDisplaced(ESI, 244714890, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 174, 138, 13, 150, 14], OperandSize::Dword)
}

fn xrstor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTOR, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 606475408, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 44, 213, 144, 20, 38, 36], OperandSize::Qword)
}

