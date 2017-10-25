use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fbld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FBLD, operand1: Some(IndirectDisplaced(BP, 51, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 102, 51], OperandSize::Word)
}

fn fbld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FBLD, operand1: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 36, 67], OperandSize::Dword)
}

fn fbld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FBLD, operand1: Some(Indirect(RAX, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 32], OperandSize::Qword)
}

