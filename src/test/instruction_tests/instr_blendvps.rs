use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blendvps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 20, 202], OperandSize::Dword)
}

#[test]
fn blendvps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1105484551, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 20, 28, 133, 7, 91, 228, 65], OperandSize::Dword)
}

#[test]
fn blendvps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 20, 241], OperandSize::Qword)
}

#[test]
fn blendvps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 20, 30], OperandSize::Qword)
}

