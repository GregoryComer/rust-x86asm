use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn subps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 92, 237], OperandSize::Dword)
}

fn subps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1216316110, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 92, 52, 141, 206, 130, 127, 72], OperandSize::Dword)
}

fn subps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 92, 223], OperandSize::Qword)
}

fn subps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1617061463, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 92, 188, 82, 87, 102, 98, 96], OperandSize::Qword)
}

