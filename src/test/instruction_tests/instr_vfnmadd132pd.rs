use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 156, 234], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 732177224, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 156, 12, 141, 72, 35, 164, 43], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 156, 211], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 156, 57], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 156, 215], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ECX, 883913654, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 156, 153, 182, 115, 175, 52], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 156, 242], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 156425724, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 156, 20, 221, 252, 221, 82, 9], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 141, 156, 211], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 2018073960, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 143, 156, 60, 181, 104, 93, 73, 120], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 155, 156, 44, 247], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 133, 142, 156, 199], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 2099098443, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 197, 129, 156, 60, 157, 75, 179, 29, 125], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 205, 148, 156, 36, 185], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 172, 156, 198], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EDI, 1148559913, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 172, 156, 135, 41, 162, 117, 68], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 191, 156, 56], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 149, 169, 156, 198], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM12)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 157, 175, 156, 3], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectDisplaced(RDX, 1847026877, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 181, 190, 156, 146, 189, 100, 23, 110], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 251, 156, 205], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 376540541, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 202, 156, 140, 138, 125, 141, 113, 22], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EBX, 1119689293, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 218, 156, 147, 77, 26, 189, 66], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM17)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 162, 253, 211, 156, 233], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM28)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 157, 194, 156, 3], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 189, 213, 156, 44, 183], OperandSize::Qword)
}

