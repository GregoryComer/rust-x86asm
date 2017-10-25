use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 150, 236], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 141511338, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 150, 44, 93, 170, 74, 111, 8], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 150, 231], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 797275807, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 150, 140, 218, 159, 118, 133, 47], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 150, 212], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 502548553, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 150, 4, 133, 73, 72, 244, 29], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 150, 205], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RCX, 1662176512, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 150, 129, 0, 205, 18, 99], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 139, 150, 243], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 1849121499, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 137, 150, 140, 241, 219, 90, 55, 110], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDX, 233567499, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 159, 150, 130, 11, 245, 235, 13], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 213, 143, 150, 237], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 197, 139, 150, 36, 176], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 451487535, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 245, 158, 150, 164, 222, 47, 39, 233, 26], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 175, 150, 198], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDI, 1642056131, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 172, 150, 151, 195, 201, 223, 97], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 5591193, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 191, 150, 148, 144, 153, 80, 85, 0], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 221, 162, 150, 211], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 1507641683, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 133, 163, 150, 132, 137, 83, 201, 220, 89], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 133, 181, 150, 4, 184], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 219, 150, 233], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 868314685, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 203, 150, 148, 194, 61, 110, 193, 51], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EDI, 1332240588, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 221, 150, 135, 204, 96, 104, 79], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 245, 146, 150, 242], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 2117652347, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 133, 204, 150, 52, 69, 123, 207, 56, 126], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 144477745, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 189, 218, 150, 148, 190, 49, 142, 156, 8], OperandSize::Qword)
}

