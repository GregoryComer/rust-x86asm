use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestnmq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 254, 15, 39, 210], OperandSize::Dword)
}

#[test]
fn vptestnmq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 512778432, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 246, 14, 39, 28, 253, 192, 96, 144, 30], OperandSize::Dword)
}

#[test]
fn vptestnmq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ESI, 78233621, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 238, 29, 39, 174, 21, 192, 169, 4], OperandSize::Dword)
}

#[test]
fn vptestnmq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 142, 11, 39, 238], OperandSize::Qword)
}

#[test]
fn vptestnmq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 230, 3, 39, 15], OperandSize::Qword)
}

#[test]
fn vptestnmq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectDisplaced(RAX, 1910746212, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 238, 19, 39, 184, 100, 172, 227, 113], OperandSize::Qword)
}

#[test]
fn vptestnmq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 254, 44, 39, 233], OperandSize::Dword)
}

#[test]
fn vptestnmq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1214940156, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 198, 47, 39, 20, 93, 252, 131, 106, 72], OperandSize::Dword)
}

#[test]
fn vptestnmq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 246, 63, 39, 20, 70], OperandSize::Dword)
}

#[test]
fn vptestnmq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 254, 42, 39, 203], OperandSize::Qword)
}

#[test]
fn vptestnmq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1581209516, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 246, 44, 39, 60, 253, 172, 87, 63, 94], OperandSize::Qword)
}

#[test]
fn vptestnmq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 1851309838, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 246, 54, 39, 188, 155, 14, 191, 88, 110], OperandSize::Qword)
}

#[test]
fn vptestnmq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 222, 73, 39, 254], OperandSize::Dword)
}

#[test]
fn vptestnmq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 331852306, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 206, 77, 39, 60, 93, 18, 170, 199, 19], OperandSize::Dword)
}

#[test]
fn vptestnmq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 214, 94, 39, 20, 118], OperandSize::Dword)
}

#[test]
fn vptestnmq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 246, 66, 39, 218], OperandSize::Qword)
}

#[test]
fn vptestnmq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 246, 77, 39, 40], OperandSize::Qword)
}

#[test]
fn vptestnmq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM25)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 182, 86, 39, 58], OperandSize::Qword)
}

