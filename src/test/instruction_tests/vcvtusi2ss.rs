use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtusi2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 118, 120, 123, 246], OperandSize::Dword)
}

fn vcvtusi2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 78, 8, 123, 46], OperandSize::Dword)
}

fn vcvtusi2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 22, 48, 123, 223], OperandSize::Qword)
}

fn vcvtusi2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM26)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 46, 0, 123, 39], OperandSize::Qword)
}

fn vcvtusi2ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(RSI)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 246, 112, 123, 230], OperandSize::Qword)
}

fn vcvtusi2ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 230888634, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 190, 8, 123, 60, 253, 186, 20, 195, 13], OperandSize::Qword)
}

