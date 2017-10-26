use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 91, 192], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 91, 14], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 91, 221], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 470740927, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 91, 188, 206, 191, 239, 14, 28], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 91, 229], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 91, 1], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 91, 202], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 91, 48], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 138, 91, 228], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 142, 91, 49], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 161, 125, 137, 91, 254], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(XMM15)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1723708562, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 125, 137, 91, 60, 213, 146, 180, 189, 102], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 91, 249], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 91, 41], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 1, 125, 170, 91, 216], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(YMM19)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 881020433, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 125, 169, 91, 28, 213, 17, 78, 131, 52], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 217, 91, 241], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 571160209, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 204, 91, 28, 157, 145, 54, 11, 34], OperandSize::Dword)
}

#[test]
fn vcvtps2dq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM27)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 125, 250, 91, 203], OperandSize::Qword)
}

#[test]
fn vcvtps2dq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2DQ, operand1: Some(Direct(ZMM17)), operand2: Some(IndirectDisplaced(RCX, 723419396, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 125, 204, 91, 137, 4, 129, 30, 43], OperandSize::Qword)
}

