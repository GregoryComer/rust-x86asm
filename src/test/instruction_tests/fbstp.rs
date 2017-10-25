use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fbstp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FBSTP, operand1: Some(IndirectDisplaced(BP, 158, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 182, 158, 0], OperandSize::Word)
}

fn fbstp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FBSTP, operand1: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 52, 247], OperandSize::Dword)
}

fn fbstp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FBSTP, operand1: Some(Indirect(RAX, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 48], OperandSize::Qword)
}

