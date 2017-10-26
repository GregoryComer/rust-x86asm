use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 151, 240], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 2007289760, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 151, 188, 131, 160, 207, 164, 119], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 151, 211], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RCX, 1704395473, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 151, 129, 209, 2, 151, 101], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 151, 226], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EDX, 768021003, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 151, 162, 11, 18, 199, 45], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 151, 209], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 2127846663, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 151, 12, 117, 7, 93, 212, 126], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 142, 151, 240], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 142, 151, 54], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EBX, 85762936, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 154, 151, 139, 120, 163, 28, 5], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 34, 117, 134, 151, 197], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RCX, 311082760, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 85, 139, 151, 145, 8, 191, 138, 18], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 625842220, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 13, 159, 151, 164, 147, 44, 152, 77, 37], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 151, 251], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 172, 151, 2], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDX, 2099799526, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 188, 151, 170, 230, 101, 40, 125], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 34, 5, 173, 151, 201], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 10777288, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 174, 151, 188, 177, 200, 114, 164, 0], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 435064894, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 45, 188, 151, 4, 181, 62, 144, 238, 25], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 191, 151, 210], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1352554703, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 206, 151, 44, 93, 207, 88, 158, 80], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 108818706, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 221, 151, 180, 137, 18, 113, 124, 6], OperandSize::Dword)
}

#[test]
fn vfmsubadd132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM13)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 82, 69, 219, 151, 229], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 77, 204, 151, 44, 158], OperandSize::Qword)
}

#[test]
fn vfmsubadd132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM27)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 37, 209, 151, 22], OperandSize::Qword)
}

