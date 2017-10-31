use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 154, 199], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 901150839, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 154, 36, 205, 119, 120, 182, 53], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 154, 194], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RCX, 2003506244, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 154, 129, 68, 20, 107, 119], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 154, 251], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 508605286, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 154, 148, 79, 102, 179, 80, 30], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 154, 193], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 154, 20, 186], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 154, 205], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EBX, 219785878, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 140, 154, 163, 150, 170, 25, 13], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ESI, 1889588650, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 156, 154, 174, 170, 213, 160, 112], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 5, 129, 154, 245], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RBX, 1146716295, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 139, 154, 147, 135, 128, 89, 68], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1442386448, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 101, 149, 154, 132, 190, 16, 18, 249, 85], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 174, 154, 192], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 835530514, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 169, 154, 188, 67, 18, 47, 205, 49], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 69, 191, 154, 4, 178], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 53, 161, 154, 237], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 333216939, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 29, 172, 154, 188, 112, 171, 124, 220, 19], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 93, 187, 154, 12, 195], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 158, 154, 252], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(ECX, 868939799, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 203, 154, 161, 23, 248, 202, 51], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 218, 154, 44, 193], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 66, 61, 182, 154, 223], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 202, 154, 62], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectDisplaced(RSI, 1440887786, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 37, 209, 154, 174, 234, 51, 226, 85], OperandSize::Qword)
}

