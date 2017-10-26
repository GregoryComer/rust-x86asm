use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blendvps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 20, 207], OperandSize::Dword)
}

#[test]
fn blendvps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EAX, 1440482848, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 20, 144, 32, 6, 220, 85], OperandSize::Dword)
}

#[test]
fn blendvps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 20, 241], OperandSize::Qword)
}

#[test]
fn blendvps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1692964391, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 20, 132, 190, 39, 150, 232, 100], OperandSize::Qword)
}

