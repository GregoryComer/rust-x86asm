use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmaxub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 222, 201], OperandSize::Dword)
}

fn vpmaxub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 1510036668, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 222, 148, 75, 188, 84, 1, 90], OperandSize::Dword)
}

fn vpmaxub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 222, 213], OperandSize::Qword)
}

fn vpmaxub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 222, 60, 86], OperandSize::Qword)
}

fn vpmaxub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 222, 205], OperandSize::Dword)
}

fn vpmaxub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 222, 14], OperandSize::Dword)
}

fn vpmaxub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 222, 241], OperandSize::Qword)
}

fn vpmaxub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1704506216, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 222, 60, 149, 104, 179, 152, 101], OperandSize::Qword)
}

fn vpmaxub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 141, 222, 228], OperandSize::Dword)
}

fn vpmaxub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EAX, 1064122088, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 138, 222, 168, 232, 54, 109, 63], OperandSize::Dword)
}

fn vpmaxub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 65, 69, 133, 222, 250], OperandSize::Qword)
}

fn vpmaxub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM16)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 125, 132, 222, 39], OperandSize::Qword)
}

fn vpmaxub_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 170, 222, 251], OperandSize::Dword)
}

fn vpmaxub_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 2136964864, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 174, 222, 140, 202, 0, 127, 95, 127], OperandSize::Dword)
}

fn vpmaxub_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 49, 45, 167, 222, 192], OperandSize::Qword)
}

fn vpmaxub_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 173, 222, 6], OperandSize::Qword)
}

fn vpmaxub_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 206, 222, 240], OperandSize::Dword)
}

fn vpmaxub_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 1641561235, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 201, 222, 172, 178, 147, 60, 216, 97], OperandSize::Dword)
}

fn vpmaxub_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 129, 29, 205, 222, 251], OperandSize::Qword)
}

fn vpmaxub_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUB, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 207, 222, 59], OperandSize::Qword)
}

