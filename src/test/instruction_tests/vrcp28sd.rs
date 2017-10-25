use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrcp28sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 157, 203, 228], OperandSize::Dword)
}

fn vrcp28sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 140, 203, 28, 206], OperandSize::Dword)
}

fn vrcp28sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 141, 158, 203, 228], OperandSize::Qword)
}

fn vrcp28sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectDisplaced(RDX, 234583014, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 181, 131, 203, 186, 230, 115, 251, 13], OperandSize::Qword)
}

