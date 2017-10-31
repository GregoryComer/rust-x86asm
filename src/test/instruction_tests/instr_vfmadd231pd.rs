use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 184, 208], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1833944695, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 184, 52, 245, 119, 198, 79, 109], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 184, 197], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 184, 58], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 184, 195], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 58143333, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 184, 12, 197, 101, 50, 119, 3], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 184, 231], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RCX, 614326511, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 184, 145, 239, 224, 157, 36], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 140, 184, 233], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1594465393, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 139, 184, 20, 85, 113, 156, 9, 95], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 2095081288, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 157, 184, 172, 130, 72, 103, 224, 124], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 221, 137, 184, 211], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Two, 330519453, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 165, 132, 184, 156, 65, 157, 83, 179, 19], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectDisplaced(RSI, 43664385, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 149, 159, 184, 166, 1, 68, 154, 2], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 173, 184, 225], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 172, 184, 62], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1452994138, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 185, 184, 60, 197, 90, 238, 154, 86], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 2, 221, 171, 184, 200], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 173, 184, 12, 142], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 530470712, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 221, 183, 184, 28, 221, 56, 87, 158, 31], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 154, 184, 213], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1174785574, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 184, 36, 245, 38, 206, 5, 70], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 220, 184, 12, 222], OperandSize::Dword)
}

#[test]
fn vfmadd231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM17)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 253, 180, 184, 233], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 1067555687, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 189, 203, 184, 164, 90, 103, 155, 161, 63], OperandSize::Qword)
}

#[test]
fn vfmadd231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 1309182741, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 223, 184, 188, 73, 21, 139, 8, 78], OperandSize::Qword)
}

