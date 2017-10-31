use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtdq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 91, 207], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 528573198, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 91, 4, 157, 14, 99, 129, 31], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 91, 194], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 91, 16], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 91, 253], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 91, 4, 122], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 91, 226], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 2017590165, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 91, 36, 253, 149, 251, 65, 120], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 141, 91, 212], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 143, 91, 55], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 124, 137, 91, 222], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1303596241, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 139, 91, 36, 125, 209, 76, 179, 77], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 169, 91, 200], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1932507823, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 174, 91, 20, 181, 175, 186, 47, 115], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 124, 174, 91, 220], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM30)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 2087536593, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 124, 169, 91, 180, 219, 209, 71, 109, 124], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 221, 91, 195], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 206, 91, 12, 147], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM24)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 1, 124, 253, 91, 248], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(ZMM24)), operand2: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 124, 202, 91, 7], OperandSize::Qword)
}

