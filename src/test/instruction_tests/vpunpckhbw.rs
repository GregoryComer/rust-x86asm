use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpunpckhbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 104, 195], OperandSize::Dword)
}

fn vpunpckhbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Eight, 2056334604, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 104, 156, 195, 12, 45, 145, 122], OperandSize::Dword)
}

fn vpunpckhbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 104, 202], OperandSize::Qword)
}

fn vpunpckhbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 104, 48], OperandSize::Qword)
}

fn vpunpckhbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 104, 239], OperandSize::Dword)
}

fn vpunpckhbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(ECX, 1546749812, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 104, 177, 116, 135, 49, 92], OperandSize::Dword)
}

fn vpunpckhbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 104, 195], OperandSize::Qword)
}

fn vpunpckhbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 104, 12, 223], OperandSize::Qword)
}

fn vpunpckhbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 143, 104, 236], OperandSize::Dword)
}

fn vpunpckhbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDI, 1566917714, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 139, 104, 135, 82, 68, 101, 93], OperandSize::Dword)
}

fn vpunpckhbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 101, 139, 104, 255], OperandSize::Qword)
}

fn vpunpckhbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RDI, 1380194164, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 142, 104, 167, 116, 23, 68, 82], OperandSize::Qword)
}

