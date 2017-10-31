use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsllvw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 141, 18, 241], OperandSize::Dword)
}

#[test]
fn vpsllvw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 139, 18, 4, 246], OperandSize::Dword)
}

#[test]
fn vpsllvw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 66, 245, 138, 18, 253], OperandSize::Qword)
}

#[test]
fn vpsllvw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 426955454, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 165, 133, 18, 20, 117, 190, 210, 114, 25], OperandSize::Qword)
}

#[test]
fn vpsllvw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 171, 18, 213], OperandSize::Dword)
}

#[test]
fn vpsllvw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1137477902, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 171, 18, 20, 221, 14, 137, 204, 67], OperandSize::Dword)
}

#[test]
fn vpsllvw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 210, 253, 162, 18, 242], OperandSize::Qword)
}

#[test]
fn vpsllvw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 157, 167, 18, 4, 94], OperandSize::Qword)
}

#[test]
fn vpsllvw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 201, 18, 232], OperandSize::Dword)
}

#[test]
fn vpsllvw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 204, 18, 20, 131], OperandSize::Dword)
}

#[test]
fn vpsllvw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 173, 207, 18, 200], OperandSize::Qword)
}

#[test]
fn vpsllvw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVW, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectDisplaced(RDX, 41119737, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 133, 199, 18, 146, 249, 111, 115, 2], OperandSize::Qword)
}

