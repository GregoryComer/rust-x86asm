use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtpd2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 90, 248], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1388751862, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 90, 12, 69, 246, 171, 198, 82], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 90, 233], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1384849917, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 90, 36, 69, 253, 33, 139, 82], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 90, 204], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 973941567, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 90, 164, 191, 63, 43, 13, 58], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 90, 205], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 844185561, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 90, 180, 79, 217, 63, 81, 50], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 90, 240], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 53643352, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 143, 90, 156, 83, 88, 136, 50, 3], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 129, 253, 141, 90, 193], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM22)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 253, 137, 90, 52, 248], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 90, 219], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 1473367759, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 90, 140, 115, 207, 206, 209, 87], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 17, 253, 171, 90, 248], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(XMM30)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 2059545020, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 253, 169, 90, 52, 125, 188, 41, 194, 122], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 249, 90, 200], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(EDI, 391924557, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 201, 90, 159, 77, 75, 92, 23], OperandSize::Dword)
}

#[test]
fn vcvtpd2ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 253, 254, 90, 228], OperandSize::Qword)
}

#[test]
fn vcvtpd2ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPD2PS, operand1: Some(Direct(YMM24)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 700174870, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 253, 202, 90, 4, 133, 22, 210, 187, 41], OperandSize::Qword)
}

