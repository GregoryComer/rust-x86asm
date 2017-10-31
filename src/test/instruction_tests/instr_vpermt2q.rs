use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermt2q_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 139, 126, 212], OperandSize::Dword)
}

#[test]
fn vpermt2q_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 138, 126, 50], OperandSize::Dword)
}

#[test]
fn vpermt2q_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 865322349, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 154, 126, 4, 133, 109, 197, 147, 51], OperandSize::Dword)
}

#[test]
fn vpermt2q_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 141, 134, 126, 255], OperandSize::Qword)
}

#[test]
fn vpermt2q_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 918931365, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 157, 137, 126, 132, 139, 165, 199, 197, 54], OperandSize::Qword)
}

#[test]
fn vpermt2q_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM22)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 205, 147, 126, 32], OperandSize::Qword)
}

#[test]
fn vpermt2q_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 171, 126, 236], OperandSize::Dword)
}

#[test]
fn vpermt2q_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 172, 126, 4, 201], OperandSize::Dword)
}

#[test]
fn vpermt2q_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EDX, 1687141574, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 189, 126, 162, 198, 188, 143, 100], OperandSize::Dword)
}

#[test]
fn vpermt2q_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 221, 161, 126, 234], OperandSize::Qword)
}

#[test]
fn vpermt2q_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 555608431, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 165, 161, 126, 180, 130, 111, 233, 29, 33], OperandSize::Qword)
}

#[test]
fn vpermt2q_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 165, 182, 126, 4, 190], OperandSize::Qword)
}

#[test]
fn vpermt2q_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 207, 126, 249], OperandSize::Dword)
}

#[test]
fn vpermt2q_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 203, 126, 36, 66], OperandSize::Dword)
}

#[test]
fn vpermt2q_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 1257928538, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 223, 126, 172, 202, 90, 119, 250, 74], OperandSize::Dword)
}

#[test]
fn vpermt2q_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 130, 197, 205, 126, 252], OperandSize::Qword)
}

#[test]
fn vpermt2q_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1002832114, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 133, 198, 126, 36, 197, 242, 0, 198, 59], OperandSize::Qword)
}

#[test]
fn vpermt2q_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 141, 212, 126, 12, 127], OperandSize::Qword)
}

