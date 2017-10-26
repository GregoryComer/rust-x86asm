use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blendvpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 21, 207], OperandSize::Dword)
}

#[test]
fn blendvpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 21, 28, 184], OperandSize::Dword)
}

#[test]
fn blendvpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 21, 254], OperandSize::Qword)
}

#[test]
fn blendvpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RSI, 1213683004, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 21, 190, 60, 85, 87, 72], OperandSize::Qword)
}

