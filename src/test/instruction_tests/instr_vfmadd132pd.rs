use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 152, 199], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 152, 34], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 152, 198], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1433475039, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 152, 36, 133, 223, 23, 113, 85], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 152, 200], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 152, 20, 185], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 152, 196], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 152, 6], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 137, 152, 220], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1599613240, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 140, 152, 60, 133, 56, 41, 88, 95], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 156, 152, 36, 89], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 146, 253, 132, 152, 201], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 139, 152, 36, 246], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM20)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 221, 146, 152, 35], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 152, 225], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EAX, 2087813290, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 173, 152, 176, 170, 128, 113, 124], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 189, 152, 47], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 171, 152, 238], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 270087357, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 245, 161, 152, 4, 213, 189, 52, 25, 16], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RAX, 1772047962, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 181, 181, 152, 168, 90, 78, 159, 105], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 188, 152, 232], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 204, 152, 2], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 221, 152, 44, 194], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 141, 177, 152, 196], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 1413172579, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 141, 195, 152, 132, 143, 99, 77, 59, 84], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 34260652, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 181, 218, 152, 28, 157, 172, 198, 10, 2], OperandSize::Qword)
}

