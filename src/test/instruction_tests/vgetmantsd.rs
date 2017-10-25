use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vgetmantsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 229, 153, 39, 219, 95], OperandSize::Dword)
}

fn vgetmantsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 1859478516, Some(OperandSize::Qword), None)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 205, 142, 39, 180, 154, 244, 99, 213, 110, 83], OperandSize::Dword)
}

fn vgetmantsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 181, 150, 39, 224, 36], OperandSize::Qword)
}

fn vgetmantsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Qword), None)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 157, 140, 39, 28, 112, 125], OperandSize::Qword)
}

