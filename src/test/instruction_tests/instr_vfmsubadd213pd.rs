use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 167, 211], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EAX, 199987197, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 167, 136, 253, 143, 235, 11], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 167, 238], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1068265154, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 167, 12, 205, 194, 110, 172, 63], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 167, 195], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 919613559, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 167, 4, 213, 119, 48, 208, 54], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 167, 219], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RBX, 916216611, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 167, 131, 35, 91, 156, 54], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 142, 167, 212], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 167, 60, 73], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ECX, 1794449466, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 153, 167, 177, 58, 32, 245, 106], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 149, 140, 167, 248], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectDisplaced(RCX, 1574870997, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 133, 137, 167, 177, 213, 159, 222, 93], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1007938852, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 245, 150, 167, 60, 133, 36, 237, 19, 60], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 173, 167, 221], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 1774296136, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 173, 167, 180, 79, 72, 156, 193, 105], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EBX, 546890377, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 190, 167, 179, 137, 226, 152, 32], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 189, 173, 167, 222], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM8)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 189, 174, 167, 63], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 165, 187, 167, 4, 211], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 188, 167, 218], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EDI, 1382916337, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 203, 167, 167, 241, 160, 109, 82], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 221, 167, 10], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 245, 146, 167, 197], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM16)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 193, 167, 50], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectDisplaced(RCX, 214874230, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 133, 220, 167, 145, 118, 184, 206, 12], OperandSize::Qword)
}

