use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fbstp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FBSTP, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 30324, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 177, 116, 118], OperandSize::Word)
}

#[test]
fn fbstp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FBSTP, operand1: Some(IndirectScaledDisplaced(EDX, Four, 841109340, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 52, 149, 92, 79, 34, 50], OperandSize::Dword)
}

#[test]
fn fbstp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FBSTP, operand1: Some(IndirectDisplaced(RBX, 803006871, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 179, 151, 233, 220, 47], OperandSize::Qword)
}

