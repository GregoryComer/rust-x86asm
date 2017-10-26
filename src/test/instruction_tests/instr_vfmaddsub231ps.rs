use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 182, 228], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 1637850332, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 182, 140, 216, 220, 156, 159, 97], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 182, 218], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 1444331235, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 182, 172, 73, 227, 190, 22, 86], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 182, 247], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 182, 28, 95], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 182, 211], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 182, 56], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 141, 182, 216], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDX, 317465619, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 139, 182, 130, 19, 36, 236, 18], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 156, 182, 20, 112], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 69, 143, 182, 230], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 117, 141, 182, 52, 83], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 1627170402, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 101, 157, 182, 156, 185, 98, 166, 252, 96], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 173, 182, 245], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 1846463454, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 172, 182, 132, 120, 222, 203, 14, 110], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EBX, 1755343823, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 186, 182, 155, 207, 107, 160, 104], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 93, 169, 182, 192], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM21)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 85, 166, 182, 49], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 1047589465, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 21, 181, 182, 148, 86, 89, 242, 112, 62], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 217, 182, 192], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 54976872, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 206, 182, 140, 206, 104, 225, 70, 3], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 221, 182, 9], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 156, 182, 230], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1496414663, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 45, 203, 182, 20, 213, 199, 121, 49, 89], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 274892790, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 5, 218, 182, 164, 151, 246, 135, 98, 16], OperandSize::Qword)
}

