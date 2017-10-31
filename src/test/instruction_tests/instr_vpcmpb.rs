use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 85, 14, 63, 248, 43], OperandSize::Dword)
}

#[test]
fn vpcmpb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1057557355, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(67)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 109, 10, 63, 52, 221, 107, 11, 9, 63, 67], OperandSize::Dword)
}

#[test]
fn vpcmpb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 53, 2, 63, 250, 22], OperandSize::Qword)
}

#[test]
fn vpcmpb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM25)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 53, 6, 63, 9, 47], OperandSize::Qword)
}

#[test]
fn vpcmpb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 109, 47, 63, 236, 37], OperandSize::Dword)
}

#[test]
fn vpcmpb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ECX, 406684370, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 117, 46, 63, 153, 210, 130, 61, 24, 21], OperandSize::Dword)
}

#[test]
fn vpcmpb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM20)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 179, 29, 43, 63, 212, 104], OperandSize::Qword)
}

#[test]
fn vpcmpb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(12)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 69, 45, 63, 8, 12], OperandSize::Qword)
}

#[test]
fn vpcmpb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 69, 74, 63, 222, 69], OperandSize::Dword)
}

#[test]
fn vpcmpb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 93, 76, 63, 28, 130, 2], OperandSize::Dword)
}

#[test]
fn vpcmpb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 85, 66, 63, 226, 37], OperandSize::Qword)
}

#[test]
fn vpcmpb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1038011, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(61)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 85, 71, 63, 12, 157, 187, 214, 15, 0, 61], OperandSize::Qword)
}

