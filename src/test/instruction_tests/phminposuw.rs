use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn phminposuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHMINPOSUW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 65, 220], OperandSize::Dword)
}

fn phminposuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHMINPOSUW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 572675103, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 65, 148, 91, 31, 84, 34, 34], OperandSize::Dword)
}

fn phminposuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHMINPOSUW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 65, 217], OperandSize::Qword)
}

fn phminposuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHMINPOSUW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 65, 4, 146], OperandSize::Qword)
}

