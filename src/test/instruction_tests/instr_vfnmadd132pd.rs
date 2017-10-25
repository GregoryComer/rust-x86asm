use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 156, 252], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 156, 44, 142], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 156, 202], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 156, 20, 191], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 156, 240], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 412264261, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 156, 52, 197, 69, 167, 146, 24], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 156, 196], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RDX, 45990202, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 156, 162, 58, 193, 189, 2], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 138, 156, 223], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 143, 156, 6], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EDI, 1263053273, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 237, 156, 156, 167, 217, 169, 72, 75], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 221, 138, 156, 223], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 154461997, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 173, 131, 156, 148, 222, 45, 231, 52, 9], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 229, 154, 156, 44, 210], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 170, 156, 193], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EAX, 426931901, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 175, 156, 160, 189, 118, 114, 25], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 185, 156, 25], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 130, 157, 172, 156, 230], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 891024729, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 165, 166, 156, 12, 85, 89, 245, 27, 53], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 245, 191, 156, 36, 65], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 219, 156, 237], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EAX, 1824351845, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 206, 156, 184, 101, 102, 189, 108], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 816086050, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 205, 219, 156, 36, 77, 34, 124, 164, 48], OperandSize::Dword)
}

#[test]
fn vfnmadd132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 237, 177, 156, 230], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 133, 199, 156, 12, 129], OperandSize::Qword)
}

#[test]
fn vfnmadd132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 181, 219, 156, 60, 255], OperandSize::Qword)
}

