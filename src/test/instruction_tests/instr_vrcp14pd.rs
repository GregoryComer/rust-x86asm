use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp14pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 76, 222], OperandSize::Dword)
}

#[test]
fn vrcp14pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EBX, 315897630, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 143, 76, 163, 30, 55, 212, 18], OperandSize::Dword)
}

#[test]
fn vrcp14pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 159, 76, 34], OperandSize::Dword)
}

#[test]
fn vrcp14pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 253, 142, 76, 218], OperandSize::Qword)
}

#[test]
fn vrcp14pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM20)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 253, 143, 76, 36, 195], OperandSize::Qword)
}

#[test]
fn vrcp14pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(XMM26)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 132170308, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 253, 153, 76, 20, 181, 68, 194, 224, 7], OperandSize::Qword)
}

#[test]
fn vrcp14pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 76, 230], OperandSize::Dword)
}

#[test]
fn vrcp14pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 76, 8], OperandSize::Dword)
}

#[test]
fn vrcp14pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 99325918, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 185, 76, 140, 94, 222, 151, 235, 5], OperandSize::Dword)
}

#[test]
fn vrcp14pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 50, 253, 172, 76, 248], OperandSize::Qword)
}

#[test]
fn vrcp14pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 1006137397, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 76, 188, 223, 53, 112, 248, 59], OperandSize::Qword)
}

#[test]
fn vrcp14pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 185, 76, 50], OperandSize::Qword)
}

#[test]
fn vrcp14pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 76, 210], OperandSize::Dword)
}

#[test]
fn vrcp14pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 702121749, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 76, 44, 157, 21, 135, 217, 41], OperandSize::Dword)
}

#[test]
fn vrcp14pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 220, 76, 12, 145], OperandSize::Dword)
}

#[test]
fn vrcp14pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 253, 207, 76, 205], OperandSize::Qword)
}

#[test]
fn vrcp14pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM12)), operand2: Some(IndirectDisplaced(RCX, 1604747587, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 253, 205, 76, 161, 67, 129, 166, 95], OperandSize::Qword)
}

#[test]
fn vrcp14pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14PD, operand1: Some(Direct(ZMM24)), operand2: Some(IndirectDisplaced(RBX, 299410551, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 253, 217, 76, 131, 119, 164, 216, 17], OperandSize::Qword)
}

