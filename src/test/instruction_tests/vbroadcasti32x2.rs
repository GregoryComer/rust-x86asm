use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vbroadcasti32x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 89, 224], OperandSize::Dword)
}

fn vbroadcasti32x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EDI, 365163676, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 89, 183, 156, 244, 195, 21], OperandSize::Dword)
}

fn vbroadcasti32x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 194, 125, 142, 89, 215], OperandSize::Qword)
}

fn vbroadcasti32x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(XMM13)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 146793804, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 140, 89, 172, 254, 76, 229, 191, 8], OperandSize::Qword)
}

fn vbroadcasti32x2_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 89, 219], OperandSize::Dword)
}

fn vbroadcasti32x2_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 89, 60, 145], OperandSize::Dword)
}

fn vbroadcasti32x2_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(YMM10)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 125, 172, 89, 211], OperandSize::Qword)
}

fn vbroadcasti32x2_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(YMM14)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 125, 174, 89, 55], OperandSize::Qword)
}

fn vbroadcasti32x2_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 89, 230], OperandSize::Dword)
}

fn vbroadcasti32x2_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectDisplaced(EAX, 1570617351, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 89, 128, 7, 184, 157, 93], OperandSize::Dword)
}

fn vbroadcasti32x2_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 66, 125, 206, 89, 202], OperandSize::Qword)
}

fn vbroadcasti32x2_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(ZMM31)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 125, 202, 89, 58], OperandSize::Qword)
}

