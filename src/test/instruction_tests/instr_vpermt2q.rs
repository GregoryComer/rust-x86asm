use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermt2q_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 140, 126, 204], OperandSize::Dword)
}

#[test]
fn vpermt2q_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 693504979, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 138, 126, 172, 126, 211, 11, 86, 41], OperandSize::Dword)
}

#[test]
fn vpermt2q_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1187703996, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 158, 126, 140, 114, 188, 236, 202, 70], OperandSize::Dword)
}

#[test]
fn vpermt2q_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 237, 143, 126, 255], OperandSize::Qword)
}

#[test]
fn vpermt2q_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 2125356300, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 157, 132, 126, 132, 194, 12, 93, 174, 126], OperandSize::Qword)
}

#[test]
fn vpermt2q_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM30)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 141, 149, 126, 49], OperandSize::Qword)
}

#[test]
fn vpermt2q_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 169, 126, 248], OperandSize::Dword)
}

#[test]
fn vpermt2q_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 169, 126, 60, 249], OperandSize::Dword)
}

#[test]
fn vpermt2q_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1651549280, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 190, 126, 28, 77, 96, 164, 112, 98], OperandSize::Dword)
}

#[test]
fn vpermt2q_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 165, 167, 126, 199], OperandSize::Qword)
}

#[test]
fn vpermt2q_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectDisplaced(RCX, 232380323, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 165, 126, 169, 163, 215, 217, 13], OperandSize::Qword)
}

#[test]
fn vpermt2q_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 1743236756, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 245, 178, 126, 188, 222, 148, 174, 231, 103], OperandSize::Qword)
}

#[test]
fn vpermt2q_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 206, 126, 255], OperandSize::Dword)
}

#[test]
fn vpermt2q_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EBX, 1757593961, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 204, 126, 163, 105, 193, 194, 104], OperandSize::Dword)
}

#[test]
fn vpermt2q_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 205, 219, 126, 44, 95], OperandSize::Dword)
}

#[test]
fn vpermt2q_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 34, 165, 198, 126, 212], OperandSize::Qword)
}

#[test]
fn vpermt2q_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 1602750927, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 253, 202, 126, 156, 223, 207, 9, 136, 95], OperandSize::Qword)
}

#[test]
fn vpermt2q_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2Q, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 181, 221, 126, 12, 251], OperandSize::Qword)
}

