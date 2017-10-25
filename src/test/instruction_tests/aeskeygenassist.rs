use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn aeskeygenassist_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESKEYGENASSIST, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 223, 212, 64], OperandSize::Dword)
}

fn aeskeygenassist_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESKEYGENASSIST, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 2131283371, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 223, 60, 93, 171, 205, 8, 127, 54], OperandSize::Dword)
}

fn aeskeygenassist_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESKEYGENASSIST, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 223, 211, 110], OperandSize::Qword)
}

fn aeskeygenassist_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESKEYGENASSIST, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 750041396, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 223, 172, 137, 52, 185, 180, 44, 62], OperandSize::Qword)
}

