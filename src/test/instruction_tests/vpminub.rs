use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpminub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 218, 240], OperandSize::Dword)
}

fn vpminub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 218, 60, 89], OperandSize::Dword)
}

fn vpminub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 218, 201], OperandSize::Qword)
}

fn vpminub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 19232777, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 218, 36, 77, 9, 120, 37, 1], OperandSize::Qword)
}

fn vpminub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 218, 192], OperandSize::Dword)
}

fn vpminub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EBX, 369489512, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 218, 179, 104, 246, 5, 22], OperandSize::Dword)
}

fn vpminub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 218, 233], OperandSize::Qword)
}

fn vpminub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 218, 44, 185], OperandSize::Qword)
}

fn vpminub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 142, 218, 249], OperandSize::Dword)
}

fn vpminub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ESI, 802455440, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 138, 218, 174, 144, 127, 212, 47], OperandSize::Dword)
}

fn vpminub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 132, 218, 208], OperandSize::Qword)
}

fn vpminub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectDisplaced(RDI, 1294596569, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 69, 134, 218, 183, 217, 249, 41, 77], OperandSize::Qword)
}

fn vpminub_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 173, 218, 247], OperandSize::Dword)
}

fn vpminub_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 174, 218, 28, 152], OperandSize::Dword)
}

fn vpminub_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 129, 37, 167, 218, 212], OperandSize::Qword)
}

fn vpminub_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 93, 167, 218, 4, 199], OperandSize::Qword)
}

fn vpminub_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 206, 218, 230], OperandSize::Dword)
}

fn vpminub_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EAX, 733890850, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 203, 218, 176, 34, 73, 190, 43], OperandSize::Dword)
}

fn vpminub_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 193, 93, 198, 218, 237], OperandSize::Qword)
}

fn vpminub_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 319135032, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 198, 218, 140, 191, 56, 157, 5, 19], OperandSize::Qword)
}

