use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vexpandps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 136, 226], OperandSize::Dword)
}

#[test]
fn vexpandps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 258935717, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 136, 188, 217, 165, 11, 111, 15], OperandSize::Dword)
}

#[test]
fn vexpandps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 125, 142, 136, 243], OperandSize::Qword)
}

#[test]
fn vexpandps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(XMM13)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1481465892, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 139, 136, 44, 157, 36, 96, 77, 88], OperandSize::Qword)
}

#[test]
fn vexpandps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 136, 228], OperandSize::Dword)
}

#[test]
fn vexpandps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 136, 44, 241], OperandSize::Dword)
}

#[test]
fn vexpandps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 125, 175, 136, 242], OperandSize::Qword)
}

#[test]
fn vexpandps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(RDI, 1111258308, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 136, 167, 196, 116, 60, 66], OperandSize::Qword)
}

#[test]
fn vexpandps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 136, 195], OperandSize::Dword)
}

#[test]
fn vexpandps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1107364436, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 136, 20, 205, 84, 10, 1, 66], OperandSize::Dword)
}

#[test]
fn vexpandps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 125, 205, 136, 247], OperandSize::Qword)
}

#[test]
fn vexpandps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 136, 41], OperandSize::Qword)
}

