use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movlps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPS, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 18, 14], OperandSize::Dword)
}

#[test]
fn movlps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RBX, 1191430137, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 18, 179, 249, 199, 3, 71], OperandSize::Qword)
}

#[test]
fn movlps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPS, operand1: Some(IndirectScaledDisplaced(EAX, Four, 2139624356, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 19, 44, 133, 164, 19, 136, 127], OperandSize::Dword)
}

#[test]
fn movlps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPS, operand1: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 19, 20, 185], OperandSize::Qword)
}

