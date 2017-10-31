use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 156, 240], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDX, 216569177, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 156, 178, 89, 149, 232, 12], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 156, 201], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 156, 52, 240], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 156, 239], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 156, 57], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 156, 211], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 156, 28, 150], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 139, 156, 222], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 2113065401, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 139, 156, 20, 141, 185, 209, 242, 125], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 571878126, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 158, 156, 12, 197, 238, 42, 22, 34], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 165, 141, 156, 249], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RAX, 1024592079, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 141, 156, 160, 207, 8, 18, 61], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 205, 155, 156, 28, 248], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 173, 156, 243], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1324429839, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 173, 156, 60, 125, 15, 50, 241, 78], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDI, 1398758551, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 188, 156, 135, 151, 92, 95, 83], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 181, 175, 156, 199], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 1341662188, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 157, 164, 156, 148, 81, 236, 35, 248, 79], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 189, 177, 156, 4, 222], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 155, 156, 235], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(ESI, 1556946815, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 202, 156, 182, 127, 31, 205, 92], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 731310992, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 218, 156, 140, 64, 144, 235, 150, 43], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 213, 251, 156, 223], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(RAX, 1610053858, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 245, 204, 156, 144, 226, 120, 247, 95], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 157, 221, 156, 4, 150], OperandSize::Qword)
}

