use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpunpckhwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 105, 219], OperandSize::Dword)
}

fn vpunpckhwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ECX, 1824036428, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 105, 177, 76, 150, 184, 108], OperandSize::Dword)
}

fn vpunpckhwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 105, 227], OperandSize::Qword)
}

fn vpunpckhwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 870372762, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 105, 44, 125, 154, 213, 224, 51], OperandSize::Qword)
}

fn vpunpckhwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 105, 200], OperandSize::Dword)
}

fn vpunpckhwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 105, 44, 120], OperandSize::Dword)
}

fn vpunpckhwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 105, 223], OperandSize::Qword)
}

fn vpunpckhwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RBX, 1538747633, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 105, 171, 241, 108, 183, 91], OperandSize::Qword)
}

fn vpunpckhwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 142, 105, 198], OperandSize::Dword)
}

fn vpunpckhwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EDX, 72589390, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 141, 105, 170, 78, 160, 83, 4], OperandSize::Dword)
}

fn vpunpckhwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 17, 61, 143, 105, 253], OperandSize::Qword)
}

fn vpunpckhwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM29)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 21, 129, 105, 19], OperandSize::Qword)
}

