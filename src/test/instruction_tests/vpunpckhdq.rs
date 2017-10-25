use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpunpckhdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 106, 217], OperandSize::Dword)
}

fn vpunpckhdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1805554804, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 106, 28, 141, 116, 148, 158, 107], OperandSize::Dword)
}

fn vpunpckhdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 106, 253], OperandSize::Qword)
}

fn vpunpckhdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 1634708413, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 106, 140, 185, 189, 171, 111, 97], OperandSize::Qword)
}

fn vpunpckhdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 106, 243], OperandSize::Dword)
}

fn vpunpckhdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 106, 40], OperandSize::Dword)
}

fn vpunpckhdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 106, 208], OperandSize::Qword)
}

fn vpunpckhdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RDX, 1591394334, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 106, 186, 30, 192, 218, 94], OperandSize::Qword)
}

fn vpunpckhdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 139, 106, 252], OperandSize::Dword)
}

fn vpunpckhdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 140, 106, 4, 183], OperandSize::Dword)
}

fn vpunpckhdq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 101, 156, 106, 44, 210], OperandSize::Dword)
}

fn vpunpckhdq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 45, 134, 106, 252], OperandSize::Qword)
}

fn vpunpckhdq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 3633616, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 93, 139, 106, 156, 131, 208, 113, 55, 0], OperandSize::Qword)
}

fn vpunpckhdq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 93, 155, 106, 36, 130], OperandSize::Qword)
}

