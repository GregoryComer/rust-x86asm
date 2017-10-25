use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrangess_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(29)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 85, 156, 81, 202, 29], OperandSize::Dword)
}

fn vrangess_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 1703598424, Some(OperandSize::Dword), None)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 117, 143, 81, 164, 190, 88, 217, 138, 101, 69], OperandSize::Dword)
}

fn vrangess_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM16)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 179, 69, 159, 81, 208, 9], OperandSize::Qword)
}

fn vrangess_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 639583792, Some(OperandSize::Dword), None)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 99, 53, 142, 81, 52, 245, 48, 70, 31, 38, 0], OperandSize::Qword)
}

