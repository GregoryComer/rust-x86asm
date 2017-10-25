use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpunpckldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 98, 231], OperandSize::Dword)
}

fn vpunpckldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 779792231, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 98, 172, 241, 103, 175, 122, 46], OperandSize::Dword)
}

fn vpunpckldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 98, 207], OperandSize::Qword)
}

fn vpunpckldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RCX, 1327622896, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 98, 185, 240, 234, 33, 79], OperandSize::Qword)
}

fn vpunpckldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 98, 219], OperandSize::Dword)
}

fn vpunpckldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDX, 323175495, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 98, 186, 71, 68, 67, 19], OperandSize::Dword)
}

fn vpunpckldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 98, 195], OperandSize::Qword)
}

fn vpunpckldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 198637665, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 98, 148, 183, 97, 248, 214, 11], OperandSize::Qword)
}

fn vpunpckldq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 138, 98, 252], OperandSize::Dword)
}

fn vpunpckldq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1893796813, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 98, 12, 245, 205, 11, 225, 112], OperandSize::Dword)
}

fn vpunpckldq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 77, 154, 98, 4, 193], OperandSize::Dword)
}

fn vpunpckldq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 117, 133, 98, 198], OperandSize::Qword)
}

fn vpunpckldq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM28)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 29, 133, 98, 6], OperandSize::Qword)
}

fn vpunpckldq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLDQ, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM23)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 69, 146, 98, 43], OperandSize::Qword)
}

