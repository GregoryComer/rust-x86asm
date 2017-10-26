use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 188, 252], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 188, 39], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 188, 215], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 188, 52, 112], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 188, 246], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDX, 1708922115, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 188, 178, 3, 21, 220, 101], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 188, 230], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RAX, 1317026852, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 188, 144, 36, 60, 128, 78], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 188, 254], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 1267610917, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 139, 188, 164, 246, 37, 53, 142, 75], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 160688241, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 156, 188, 140, 183, 113, 232, 147, 9], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 21, 139, 188, 193], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 1404037254, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 93, 138, 188, 180, 151, 134, 232, 175, 83], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 364216391, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 159, 188, 148, 120, 71, 128, 181, 21], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 170, 188, 226], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDX, 83555575, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 188, 154, 247, 244, 250, 4], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 605988821, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 187, 188, 156, 126, 213, 167, 30, 36], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 21, 163, 188, 248], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectDisplaced(RBX, 455658630, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 162, 188, 155, 134, 204, 40, 27], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 427358616, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 77, 187, 188, 156, 177, 152, 249, 120, 25], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 153, 188, 230], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 459943546, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 204, 188, 148, 75, 122, 46, 106, 27], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 217, 188, 52, 218], OperandSize::Dword)
}

#[test]
fn vfnmadd231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 66, 85, 180, 188, 200], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 590422929, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 77, 202, 188, 156, 83, 145, 35, 49, 35], OperandSize::Qword)
}

#[test]
fn vfnmadd231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 1967242295, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 218, 188, 140, 223, 55, 188, 65, 117], OperandSize::Qword)
}

