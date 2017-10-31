use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blendvps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 20, 228], OperandSize::Dword)
}

#[test]
fn blendvps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPS, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 20, 40], OperandSize::Dword)
}

#[test]
fn blendvps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 20, 197], OperandSize::Qword)
}

#[test]
fn blendvps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPS, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 20, 15], OperandSize::Qword)
}

