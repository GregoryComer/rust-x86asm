use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 154, 240], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ECX, 973033678, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 154, 137, 206, 80, 255, 57], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 154, 252], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RDX, 983032949, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 154, 170, 117, 228, 151, 58], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 154, 211], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 154, 60, 138], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 154, 223], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 222223880, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 154, 140, 152, 8, 222, 62, 13], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 143, 154, 254], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ECX, 2144277548, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 143, 154, 145, 44, 20, 207, 127], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 156, 154, 34], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 13, 137, 154, 196], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 109, 133, 154, 28, 184], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 359165365, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 117, 159, 154, 52, 197, 181, 109, 104, 21], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 173, 154, 247], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 404408087, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 170, 154, 28, 117, 23, 199, 26, 24], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 33309449, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 190, 154, 188, 198, 9, 67, 252, 1], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 37, 167, 154, 195], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 105055492, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 13, 169, 154, 36, 221, 4, 5, 67, 6], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 821761224, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 53, 181, 154, 12, 189, 200, 20, 251, 48], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 158, 154, 234], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EDI, 1230812923, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 207, 154, 183, 251, 182, 92, 73], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(ESI, 498134393, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 219, 154, 174, 121, 237, 176, 29], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 214, 154, 205], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 201, 154, 38], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 1554545079, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 101, 221, 154, 148, 247, 183, 121, 168, 92], OperandSize::Qword)
}

