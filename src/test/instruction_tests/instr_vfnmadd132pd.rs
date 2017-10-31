use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 156, 240], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ECX, 1182668014, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 156, 161, 238, 20, 126, 70], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 156, 213], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 156, 55], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 156, 244], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 156, 26], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 156, 248], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 156, 31], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 137, 156, 228], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 1644411737, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 140, 156, 188, 134, 89, 187, 3, 98], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 156, 156, 60, 70], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 157, 130, 156, 214], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 141, 130, 156, 60, 71], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 149, 145, 156, 52, 209], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 174, 156, 254], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 231251234, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 172, 156, 20, 149, 34, 157, 200, 13], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 185, 156, 20, 154], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 173, 162, 156, 208], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1051162009, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 165, 167, 156, 52, 189, 153, 117, 167, 62], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1539794040, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 181, 183, 156, 36, 141, 120, 100, 199, 91], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 252, 156, 255], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 1859905701, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 203, 156, 188, 138, 165, 232, 219, 110], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EDI, 321710840, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 219, 156, 191, 248, 234, 44, 19], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 149, 214, 156, 245], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM22)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 205, 196, 156, 59], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(RAX, 749864743, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 253, 218, 156, 144, 39, 7, 178, 44], OperandSize::Qword)
}

