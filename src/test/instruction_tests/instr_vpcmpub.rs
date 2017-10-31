use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 101, 15, 62, 228, 93], OperandSize::Dword)
}

#[test]
fn vpcmpub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 69, 10, 62, 56, 18], OperandSize::Dword)
}

#[test]
fn vpcmpub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(79)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 45, 9, 62, 255, 79], OperandSize::Qword)
}

#[test]
fn vpcmpub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM9)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 53, 11, 62, 32, 18], OperandSize::Qword)
}

#[test]
fn vpcmpub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(16)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 117, 43, 62, 239, 16], OperandSize::Dword)
}

#[test]
fn vpcmpub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EBX, 455237098, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 69, 41, 62, 147, 234, 93, 34, 27, 13], OperandSize::Dword)
}

#[test]
fn vpcmpub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM19)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 179, 29, 35, 62, 227, 74], OperandSize::Qword)
}

#[test]
fn vpcmpub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 2012339439, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(26)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 61, 44, 62, 20, 133, 239, 220, 241, 119, 26], OperandSize::Qword)
}

#[test]
fn vpcmpub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 117, 78, 62, 233, 112], OperandSize::Dword)
}

#[test]
fn vpcmpub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1038204386, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 69, 77, 62, 52, 85, 226, 189, 225, 61, 98], OperandSize::Dword)
}

#[test]
fn vpcmpub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM23)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 179, 29, 78, 62, 223, 87], OperandSize::Qword)
}

#[test]
fn vpcmpub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 902889952, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 53, 66, 62, 172, 121, 224, 1, 209, 53, 6], OperandSize::Qword)
}

