use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 158, 226], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 158, 42], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 158, 238], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 920762325, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 158, 164, 254, 213, 183, 225, 54], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 158, 222], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 158, 4, 187], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 158, 233], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1098025328, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 158, 52, 85, 112, 137, 114, 65], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 141, 158, 201], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 2085905425, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 141, 158, 20, 77, 17, 100, 84, 124], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 171490843, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 154, 158, 156, 248, 27, 190, 56, 10], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 2, 149, 138, 158, 208], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 1791307536, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 237, 135, 158, 188, 119, 16, 47, 197, 106], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1662919997, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 141, 145, 158, 12, 205, 61, 37, 30, 99], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 171, 158, 213], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 2011965338, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 172, 158, 44, 181, 154, 39, 236, 119], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 890190842, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 186, 158, 188, 206, 250, 59, 15, 53], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 229, 169, 158, 212], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 504111759, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 189, 163, 158, 20, 85, 143, 34, 12, 30], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 904137565, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 182, 158, 156, 79, 93, 11, 228, 53], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 186, 158, 246], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 1113738482, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 158, 140, 206, 242, 76, 98, 66], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1478855881, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 221, 158, 60, 69, 201, 140, 37, 88], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 194, 213, 243, 158, 203], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM17)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 245, 196, 158, 11], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 1609270155, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 165, 209, 158, 148, 179, 139, 131, 235, 95], OperandSize::Qword)
}

