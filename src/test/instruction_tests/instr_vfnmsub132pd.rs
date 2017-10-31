use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 158, 217], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 158, 36, 199], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 158, 240], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 897650892, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 158, 164, 222, 204, 16, 129, 53], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 158, 237], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1179490868, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 158, 60, 69, 52, 154, 77, 70], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 158, 209], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 992765850, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 158, 44, 117, 154, 103, 44, 59], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 141, 158, 244], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 138, 158, 56], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1036306064, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 158, 158, 28, 69, 144, 198, 196, 61], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 34, 253, 131, 158, 229], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 229, 133, 158, 0], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 100629998, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 165, 149, 158, 148, 146, 238, 125, 255, 5], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 173, 158, 221], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 1577976921, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 170, 158, 188, 203, 89, 4, 14, 94], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 186, 158, 28, 254], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 213, 173, 158, 200], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 1256594179, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 141, 167, 158, 164, 135, 3, 27, 230, 74], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RDX, 1710955374, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 181, 183, 158, 130, 110, 27, 251, 101], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 252, 158, 218], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 884273316, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 207, 158, 52, 245, 164, 240, 180, 52], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 223, 158, 52, 71], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM15)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 34, 133, 222, 158, 246], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 87916892, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 221, 207, 158, 148, 193, 92, 129, 61, 5], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM26)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 173, 210, 158, 0], OperandSize::Qword)
}

