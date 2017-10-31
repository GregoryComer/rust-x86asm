use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 154, 215], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 656440563, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 154, 60, 157, 243, 124, 32, 39], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 154, 199], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 154, 0], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 154, 249], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1989215907, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 154, 172, 254, 163, 6, 145, 118], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 154, 204], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 2090092130, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 154, 4, 141, 98, 70, 148, 124], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 137, 154, 208], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDI, 1624591735, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 138, 154, 151, 119, 77, 213, 96], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 186124912, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 154, 154, 28, 213, 112, 10, 24, 11], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 133, 143, 154, 194], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 221, 139, 154, 20, 192], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 165, 156, 154, 12, 182], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 169, 154, 219], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 170, 154, 52, 127], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 763702374, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 191, 154, 12, 213, 102, 44, 133, 45], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 229, 166, 154, 226], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 223589344, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 237, 162, 154, 60, 85, 224, 179, 83, 13], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM27)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 165, 181, 154, 9], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 186, 154, 227], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 207, 154, 51], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 221, 154, 50], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 205, 177, 154, 235], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 409398098, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 165, 199, 154, 132, 223, 82, 235, 102, 24], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1563183667, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 245, 222, 154, 36, 189, 51, 74, 44, 93], OperandSize::Qword)
}

