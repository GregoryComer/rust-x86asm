use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 166, 216], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 204180435, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 166, 140, 73, 211, 139, 43, 12], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 166, 200], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1491658271, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 166, 60, 141, 31, 230, 232, 88], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 166, 207], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EBX, 2036936288, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 166, 139, 96, 46, 105, 121], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 166, 244], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RDX, 377531329, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 166, 170, 193, 171, 128, 22], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 137, 166, 238], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 142, 166, 60, 154], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDX, 1462036521, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 153, 166, 154, 41, 232, 36, 87], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 181, 129, 166, 195], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 245, 140, 166, 52, 66], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM27)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 165, 149, 166, 42], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 166, 193], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 166, 20, 144], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 1953850985, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 185, 166, 156, 65, 105, 102, 117, 116], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 229, 165, 166, 217], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 538756498, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 197, 175, 166, 28, 205, 146, 197, 28, 32], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 918003879, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 179, 166, 164, 146, 167, 160, 183, 54], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 217, 166, 202], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 1013674210, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 201, 166, 172, 255, 226, 112, 107, 60], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(ECX, 1860952690, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 218, 166, 137, 114, 226, 235, 110], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 229, 158, 166, 218], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 584635781, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 149, 194, 166, 28, 197, 133, 213, 216, 34], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1025269119, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 165, 214, 166, 20, 149, 127, 93, 28, 61], OperandSize::Qword)
}

