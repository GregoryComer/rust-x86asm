use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttps2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 91, 230], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 2020007775, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 91, 156, 73, 95, 223, 102, 120], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 91, 212], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 1512912451, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 91, 148, 178, 67, 54, 45, 90], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 91, 221], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 91, 43], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 91, 237], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 136998431, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 91, 140, 87, 31, 110, 42, 8], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 139, 91, 235], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 756764954, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 138, 91, 180, 134, 26, 81, 27, 45], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 126, 137, 91, 239], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM25)), operand2: Some(IndirectDisplaced(RDI, 626284360, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 126, 139, 91, 143, 72, 87, 84, 37], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 169, 91, 219], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(EBX, 1788362522, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 170, 91, 163, 26, 63, 152, 106], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 126, 172, 91, 237], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM21)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 1741591355, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 126, 169, 91, 172, 248, 59, 147, 206, 103], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 156, 91, 254], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1015640921, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 201, 91, 12, 221, 89, 115, 137, 60], OperandSize::Dword)
}

#[test]
fn vcvttps2dq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 126, 153, 91, 221], OperandSize::Qword)
}

#[test]
fn vcvttps2dq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 126, 202, 91, 60, 75], OperandSize::Qword)
}

