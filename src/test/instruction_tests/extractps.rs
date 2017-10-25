use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn extractps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::EXTRACTPS, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 23, 228, 32], OperandSize::Dword)
}

fn extractps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::EXTRACTPS, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 23, 31, 52], OperandSize::Dword)
}

fn extractps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::EXTRACTPS, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 23, 253, 84], OperandSize::Qword)
}

fn extractps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::EXTRACTPS, operand1: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 23, 52, 248, 46], OperandSize::Qword)
}

