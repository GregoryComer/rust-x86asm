use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 152, 241], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDX, 1440562323, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 152, 130, 147, 60, 221, 85], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 152, 238], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 152, 60, 78], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 152, 219], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EAX, 1906734256, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 152, 136, 176, 116, 166, 113], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 152, 224], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 152, 54], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 138, 152, 222], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ECX, 917190309, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 143, 152, 185, 165, 54, 171, 54], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1460858200, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 245, 153, 152, 172, 112, 88, 237, 18, 87], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 149, 130, 152, 228], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 173, 143, 152, 36, 113], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 221, 153, 152, 12, 219], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 174, 152, 200], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 2086662368, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 173, 152, 12, 93, 224, 240, 95, 124], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 188, 152, 15], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 66, 141, 170, 152, 213], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM26)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 173, 165, 152, 17], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RCX, 1951922311, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 237, 185, 152, 153, 135, 248, 87, 116], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 223, 152, 224], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 942298506, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 206, 152, 12, 181, 138, 85, 42, 56], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EBX, 1064584619, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 205, 222, 152, 179, 171, 69, 116, 63], OperandSize::Dword)
}

#[test]
fn vfmadd132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 50, 229, 242, 152, 252], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 123627034, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 157, 197, 152, 36, 221, 26, 102, 94, 7], OperandSize::Qword)
}

#[test]
fn vfmadd132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM30)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 141, 209, 152, 23], OperandSize::Qword)
}

