use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 152, 231], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 152, 18], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 152, 235], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RBX, 963555338, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 152, 139, 10, 176, 110, 57], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 152, 240], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1244920252, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 152, 60, 133, 188, 249, 51, 74], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 152, 234], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Eight, 390568360, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 152, 148, 251, 168, 153, 71, 23], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 138, 152, 211], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EBX, 279442392, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 138, 152, 187, 216, 243, 167, 16], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 309526826, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 245, 156, 152, 20, 181, 42, 1, 115, 18], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 50, 213, 133, 152, 210], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectDisplaced(RSI, 132563854, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 181, 132, 152, 150, 142, 195, 230, 7], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 782996775, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 173, 159, 152, 44, 221, 39, 149, 171, 46], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 173, 152, 223], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ESI, 2098442279, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 174, 152, 150, 39, 176, 19, 125], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 187, 152, 31], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 133, 167, 152, 197], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 172, 152, 28, 91], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 149, 187, 152, 20, 190], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 191, 152, 199], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 206, 152, 47], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 219, 152, 4, 88], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 149, 146, 152, 225], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 157, 204, 152, 4, 144], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectDisplaced(RBX, 2113149840, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 189, 215, 152, 179, 144, 27, 244, 125], OperandSize::Qword)
}

