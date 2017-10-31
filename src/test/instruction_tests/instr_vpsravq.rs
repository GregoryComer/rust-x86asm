use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsravq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 137, 70, 197], OperandSize::Dword)
}

#[test]
fn vpsravq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDI, 63578268, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 141, 70, 183, 156, 32, 202, 3], OperandSize::Dword)
}

#[test]
fn vpsravq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ECX, 127666546, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 245, 158, 70, 161, 114, 9, 156, 7], OperandSize::Dword)
}

#[test]
fn vpsravq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 141, 142, 70, 232], OperandSize::Qword)
}

#[test]
fn vpsravq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 79646469, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 142, 70, 52, 85, 5, 79, 191, 4], OperandSize::Qword)
}

#[test]
fn vpsravq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 658988846, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 237, 145, 70, 180, 203, 46, 95, 71, 39], OperandSize::Qword)
}

#[test]
fn vpsravq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 171, 70, 214], OperandSize::Dword)
}

#[test]
fn vpsravq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(ECX, 1679316447, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 170, 70, 169, 223, 85, 24, 100], OperandSize::Dword)
}

#[test]
fn vpsravq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 191, 70, 39], OperandSize::Dword)
}

#[test]
fn vpsravq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 194, 141, 174, 70, 206], OperandSize::Qword)
}

#[test]
fn vpsravq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM28)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 157, 162, 70, 47], OperandSize::Qword)
}

#[test]
fn vpsravq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 197, 185, 70, 40], OperandSize::Qword)
}

#[test]
fn vpsravq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 70, 220], OperandSize::Dword)
}

#[test]
fn vpsravq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 204, 70, 42], OperandSize::Dword)
}

#[test]
fn vpsravq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1982218424, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 222, 70, 12, 85, 184, 64, 38, 118], OperandSize::Dword)
}

#[test]
fn vpsravq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 189, 193, 70, 195], OperandSize::Qword)
}

#[test]
fn vpsravq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 396253512, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 173, 194, 70, 172, 248, 72, 89, 158, 23], OperandSize::Qword)
}

#[test]
fn vpsravq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1446862601, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 205, 215, 70, 12, 213, 9, 95, 61, 86], OperandSize::Qword)
}

