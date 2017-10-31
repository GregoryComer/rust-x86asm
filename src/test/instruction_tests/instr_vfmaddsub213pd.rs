use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 166, 235], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ECX, 1722800890, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 166, 169, 250, 218, 175, 102], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 166, 247], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1733357162, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 166, 28, 253, 106, 238, 80, 103], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 166, 204], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 825654925, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 166, 140, 94, 141, 126, 54, 49], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 166, 211], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RDX, 519407988, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 166, 130, 116, 137, 245, 30], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 137, 166, 237], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 138, 166, 54], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 245, 155, 166, 44, 121], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 34, 157, 131, 166, 199], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM29)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 149, 135, 166, 8], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM14)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 141, 159, 166, 34], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 170, 166, 218], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 174, 166, 6], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 379208424, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 187, 166, 188, 129, 232, 66, 154, 22], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 221, 170, 166, 218], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RDX, 998337218, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 221, 169, 166, 154, 194, 106, 129, 59], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1368017568, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 133, 178, 166, 28, 253, 160, 74, 138, 81], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 251, 166, 240], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 166, 51], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EAX, 1998739519, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 221, 166, 152, 63, 88, 34, 119], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 253, 217, 166, 206], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 300579322, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 189, 196, 166, 4, 253, 250, 121, 234, 17], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 222, 166, 60, 218], OperandSize::Qword)
}

