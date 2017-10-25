use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsrlvw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 143, 16, 246], OperandSize::Dword)
}

fn vpsrlvw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EBX, 8665853, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 143, 16, 155, 253, 58, 132, 0], OperandSize::Dword)
}

fn vpsrlvw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 237, 140, 16, 207], OperandSize::Qword)
}

fn vpsrlvw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 808067084, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 245, 138, 16, 60, 197, 12, 32, 42, 48], OperandSize::Qword)
}

fn vpsrlvw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 175, 16, 208], OperandSize::Dword)
}

fn vpsrlvw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(ECX, 1610769777, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 172, 16, 145, 113, 101, 2, 96], OperandSize::Dword)
}

fn vpsrlvw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 197, 172, 16, 213], OperandSize::Qword)
}

fn vpsrlvw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 165, 166, 16, 4, 179], OperandSize::Qword)
}

fn vpsrlvw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 202, 16, 243], OperandSize::Dword)
}

fn vpsrlvw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 945173377, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 203, 16, 60, 117, 129, 51, 86, 56], OperandSize::Dword)
}

fn vpsrlvw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 213, 207, 16, 226], OperandSize::Qword)
}

fn vpsrlvw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 173, 204, 16, 36, 151], OperandSize::Qword)
}

