use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtdq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 91, 222], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDX, 922610046, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 91, 162, 126, 233, 253, 54], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 91, 215], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RAX, 775571446, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 91, 176, 246, 71, 58, 46], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 91, 211], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 1493009995, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 91, 156, 151, 75, 134, 253, 88], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 91, 234], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 1184430274, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 91, 188, 95, 194, 248, 152, 70], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 143, 91, 215], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 143, 91, 20, 73], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 124, 142, 91, 212], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM14)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 124, 137, 91, 49], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 170, 91, 249], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 91, 15], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 129, 124, 170, 91, 234], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM31)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 414482160, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 124, 171, 91, 188, 70, 240, 126, 180, 24], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 189, 91, 245], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 205, 91, 62], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM31)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 129, 124, 190, 91, 231], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectDisplaced(RDI, 547503518, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 206, 91, 159, 158, 61, 162, 32], OperandSize::Qword)
}

