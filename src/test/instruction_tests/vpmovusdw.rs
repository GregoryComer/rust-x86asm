use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovusdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 140, 19, 203], OperandSize::Dword)
}

fn vpmovusdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 19, 20, 177], OperandSize::Dword)
}

fn vpmovusdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 126, 141, 19, 252], OperandSize::Qword)
}

fn vpmovusdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 1695560407, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 19, 20, 253, 215, 50, 16, 101], OperandSize::Qword)
}

fn vpmovusdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 173, 19, 252], OperandSize::Dword)
}

fn vpmovusdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 19, 52, 254], OperandSize::Dword)
}

fn vpmovusdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(XMM24)), operand2: Some(Direct(YMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 130, 126, 173, 19, 216], OperandSize::Qword)
}

fn vpmovusdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectScaledDisplaced(RDX, Four, 1046669509, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 19, 20, 149, 197, 232, 98, 62], OperandSize::Qword)
}

fn vpmovusdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 205, 19, 245], OperandSize::Dword)
}

fn vpmovusdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 507217490, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 19, 180, 241, 82, 134, 59, 30], OperandSize::Dword)
}

fn vpmovusdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(YMM27)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 126, 202, 19, 251], OperandSize::Qword)
}

fn vpmovusdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 72, 19, 20, 247], OperandSize::Qword)
}

